//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1210/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1210<F: Float>(t1955: F, t32477: F, t103059: F, t119969: F, t121887: F, t126256: F, t126260: F, t27286: F, t27354: F, t28425: F, t31812: F, t32426: F, t32450: F, t32463: F, t34044: F, t34049: F, t34050: F, t34063: F, t7083: F, t7774: F, t7779: F, t8649: F, t886: F) -> (F, F) {
    let t127739 = t1955 * t32477;
    let t127758 = F::cast_from(0.57119737665102352616e0_f64) * t32426 * t34050 + F::cast_from(0.37645955677973955999e-4_f64) * t119969 - F::cast_from(0.8673628188205199462e0_f64) * t32450 * t7779 + F::cast_from(0.8673628188205199462e0_f64) * t127739 * t27354 - F::cast_from(0.11423947533020470523e1_f64) * t121887 * t34044 - F::cast_from(0.11423947533020470523e1_f64) * t32463 * t103059 * t7774 - F::cast_from(0.11423947533020470523e1_f64) * t32463 * t28425 * t27286 + F::cast_from(0.112937867033921868e-1_f64) * t126256 + F::cast_from(0.112937867033921868e-2_f64) * t126260 - F::cast_from(0.8673628188205199462e0_f64) * t34063 * t7083 - F::cast_from(0.17135921299530705785e1_f64) * t8649 * t31812 * t34049 * t886;
    (t127739, t127758)
}
