//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1429/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1429<F: Float>(t12213: F, t1710: F, t38961: F, t723: F, t12218: F, t12220: F, t12224: F, t12228: F, t12256: F, t1445: F, t1457: F, t2049: F, t2087: F, t2197: F, t2615: F, t326: F, t33228: F, t33231: F, t33238: F, t33240: F, t33246: F, t33248: F, t33253: F, t33255: F, t39044: F, t5782: F, t7736: F, t833: F) -> (F, F, F) {
    let t39091 = t12213 * t1710;
    let t39095 = t38961 * t723;
    let t39101 = t33228 - t33231 - t33238 - t33240 - F::cast_from(0.69017266717057349418e1_f64) * t2087 * t1445 * t12218 * t1710 - F::cast_from(0.13803453343411469884e2_f64) * t5782 * t12220 + F::cast_from(0.92023022289409799224e1_f64) * t2615 * t326 * t39044 + t33246 - F::cast_from(0.10725146985555128001e1_f64) * t12256 * t7736 + t33248 - F::cast_from(0.21450293971110256002e1_f64) * t2049 * t1457 * t12224 + F::cast_from(0.11502877786176224903e2_f64) * t833 * t1445 * t39091 + F::cast_from(0.23005755572352449806e2_f64) * t833 * t1445 * t39095 + F::cast_from(0.23005755572352449806e2_f64) * t2197 * t12228 - t33253 - t33255;
    (t39091, t39095, t39101)
}
