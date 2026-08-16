//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1210/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1210(t1955: f64, t32477: f64, t103059: f64, t119969: f64, t121887: f64, t126256: f64, t126260: f64, t27286: f64, t27354: f64, t28425: f64, t31812: f64, t32426: f64, t32450: f64, t32463: f64, t34044: f64, t34049: f64, t34050: f64, t34063: f64, t7083: f64, t7774: f64, t7779: f64, t8649: f64, t886: f64) -> (f64, f64) {
    let t127739 = t1955 * t32477;
    let t127758 = 0.57119737665102352616e0_f64 * t32426 * t34050 + 0.37645955677973955999e-4_f64 * t119969 - 0.8673628188205199462e0_f64 * t32450 * t7779 + 0.8673628188205199462e0_f64 * t127739 * t27354 - 0.11423947533020470523e1_f64 * t121887 * t34044 - 0.11423947533020470523e1_f64 * t32463 * t103059 * t7774 - 0.11423947533020470523e1_f64 * t32463 * t28425 * t27286 + 0.112937867033921868e-1_f64 * t126256 + 0.112937867033921868e-2_f64 * t126260 - 0.8673628188205199462e0_f64 * t34063 * t7083 - 0.17135921299530705785e1_f64 * t8649 * t31812 * t34049 * t886;
    (t127739, t127758)
}
