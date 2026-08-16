//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 724/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk724<F: Float>(t45: F, t4716: F, t773: F, t774: F, t1364: F, t226: F, t3629: F, t2175: F, t3643: F, t2225: F, t4573: F, t4579: F, t78: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t4718 = t773 * t774 * t4716;
    let t4722 = t226 * t1364;
    let t4723 = t3629 * t4722;
    let t4724 = t2175 * t4723;
    let t4727 = F::cast_from(8.0_f64) * t3643;
    let t4733 = piecewise3::<F>(t151, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2225 * t4573 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t78 * t4579);
    (t4718, t4722, t4724, t4727, t4733)
}
