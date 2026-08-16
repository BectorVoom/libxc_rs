//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2235/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2235(t104465: f64, t105365: f64, t105530: f64, t111815: f64, t1203: f64, t1214: f64, t1774: f64, t1829: f64, t21483: f64, t21557: f64, t26949: f64, t26994: f64, t27011: f64, t29109: f64, t29160: f64, t29195: f64, t29200: f64, t29207: f64, t29213: f64, t30735: f64, t30740: f64, t30743: f64, t30866: f64, t5498: f64, t6563: f64, t6580: f64, t7627: f64, t7636: f64, t7637: f64, t7643: f64, t96954: f64, t96979: f64, t96982: f64, t96986: f64, t97050: f64, t97304: f64) -> f64 {
    let t111959 = -0.17347256376410398924e1_f64 * t96979 * t111815 * t96982 + 0.17347256376410398924e1_f64 * t96986 * t111815 * t21483 - 0.13170898365871023197e1_f64 * t104465 * t1829 + 0.8673628188205199462e0_f64 * t29200 * t29195 * t21557 - 0.8673628188205199462e0_f64 * t7636 * t7637 * t7627 * t6563 + 0.17347256376410398924e1_f64 * t105365 * t29213 + 0.17347256376410398924e1_f64 * t7643 * t7637 * t29109 * t1774 + 0.13170898365871023197e1_f64 * t27011 * t6580 + 0.34694512752820797848e1_f64 * t97304 * t30866 * t96954 + 0.34694512752820797848e1_f64 * t26994 * t7637 * t30743 * t1214 - 0.17347256376410398924e1_f64 * t105530 * t29160 - 0.26020884564615598386e1_f64 * t97050 * t30740 + 0.17347256376410398924e1_f64 * t26994 * t7637 * t30735 * t1203 - 0.26020884564615598386e1_f64 * t26949 * t7637 * t30735 * t1214 - 0.13170898365871023197e1_f64 * t29207 * t5498;
    t111959
}
