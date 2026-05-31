//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 812/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk812<F: Float>(t481: F, t7184: F, t1234: F, t2505: F, t490: F, t7088: F, t109: F, t111: F, t1536: F, t1544: F, t1547: F, t2498: F, t2504: F, t2506: F, t2527: F, t486: F, t491: F, t7165: F, t7175: F, t7181: F, t915: F, t917: F) -> F {
    let t7185 = t7184 * t481;
    let t7188 = t2505 * t1234;
    let t7191 = t490 * t7088;
    let t7194 = F::cast_from(3.0_f64) * t109 * t7191 - t7165 * t111 + F::cast_from(3.0_f64) * t1536 * t917 - F::cast_from(12.0_f64) * t915 * t1544 + F::cast_from(3.0_f64) * t915 * t1547 + F::cast_from(6.0_f64) * t2498 * t491 + F::cast_from(60.0_f64) * t2504 * t7181 - F::cast_from(24.0_f64) * t2504 * t7185 - F::cast_from(12.0_f64) * t2504 * t7188 - F::cast_from(24.0_f64) * t7175 * t2506 + F::cast_from(6.0_f64) * t486 * t2527;
    t7194
}
