//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 998/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk998<F: Float>(t12000: F, t493: F, t492: F, t11986: F, t550: F, t1365: F, t10184: F, t10187: F, t10195: F, t10198: F, t10229: F, t10236: F, t10238: F, t10240: F, t10245: F, t105: F, t1358: F, t3692: F, t419: F, t9207: F, t9210: F) -> (F, F, F, F, F) {
    let t12001 = t493 * t12000;
    let t12002 = t492 * t12001;
    let t12007 = t550 * t11986;
    let t12008 = t1365 * t12007;
    let t12011 = t10184 + t10187 - F::cast_from(0.28455006635676149599e-1_f64) * t105 * t12002 + F::cast_from(0.28455006635676149599e-1_f64) * t419 * t3692 + F::cast_from(0.31616674039640166221e-2_f64) * t1358 * t12008 - t10195 - t10198 - t9207 + t9210 + t10229 + t10236 - t10238 - t10240 + t10245;
    (t12001, t12002, t12007, t12008, t12011)
}
