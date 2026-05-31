//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1120/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1120<F: Float>(t2394: F, t33: F, t2411: F, t14365: F, t1113: F, t775: F, t2430: F, t2408: F, t890: F, t2832: F, t1940: F, t1963: F, t2403: F, t25206: F, t25436: F, t25440: F, t25445: F, t3351: F, t4541: F, t7087: F, t7091: F, t7200: F, t7207: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25752 = t33 * t2394;
    let t25759 = t2411 * t33;
    let t25760 = t25759 * t14365;
    let t25763 = t1113 * t775;
    let t25767 = t33 * t2430;
    let t25778 = t33 * t2408;
    let t25781 = t1113 * t890;
    let t25784 = t33 * t2832;
    let t25791 = F::cast_from(3.0_f64) * t4541 * t1963 * t25752 + F::cast_from(3.0_f64) * t2403 * t7087 * t7200 - F::cast_from(3.0_f64) * t25206 * t25760 + F::cast_from(3.0_f64) * t2403 * t1963 * t25763 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t1963 * t25767 + t1940 * t25436 * t33 / F::cast_from(2.0_f64) - t1940 * t25440 * t7207 + t1940 * t7087 * t1113 + t1940 * t25445 * t25778 - t1940 * t7091 * t25781 - t1940 * t7091 * t25784 / F::cast_from(2.0_f64) + t1940 * t1963 * t3351 / F::cast_from(2.0_f64);
    (t25752, t25759, t25760, t25763, t25767, t25778, t25781, t25784, t25791)
}
