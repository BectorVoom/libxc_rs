//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2235/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2235<F: Float>(t104465: F, t105365: F, t105530: F, t111815: F, t1203: F, t1214: F, t1774: F, t1829: F, t21483: F, t21557: F, t26949: F, t26994: F, t27011: F, t29109: F, t29160: F, t29195: F, t29200: F, t29207: F, t29213: F, t30735: F, t30740: F, t30743: F, t30866: F, t5498: F, t6563: F, t6580: F, t7627: F, t7636: F, t7637: F, t7643: F, t96954: F, t96979: F, t96982: F, t96986: F, t97050: F, t97304: F) -> F {
    let t111959 = -F::cast_from(0.17347256376410398924e1_f64) * t96979 * t111815 * t96982 + F::cast_from(0.17347256376410398924e1_f64) * t96986 * t111815 * t21483 - F::cast_from(0.13170898365871023197e1_f64) * t104465 * t1829 + F::cast_from(0.8673628188205199462e0_f64) * t29200 * t29195 * t21557 - F::cast_from(0.8673628188205199462e0_f64) * t7636 * t7637 * t7627 * t6563 + F::cast_from(0.17347256376410398924e1_f64) * t105365 * t29213 + F::cast_from(0.17347256376410398924e1_f64) * t7643 * t7637 * t29109 * t1774 + F::cast_from(0.13170898365871023197e1_f64) * t27011 * t6580 + F::cast_from(0.34694512752820797848e1_f64) * t97304 * t30866 * t96954 + F::cast_from(0.34694512752820797848e1_f64) * t26994 * t7637 * t30743 * t1214 - F::cast_from(0.17347256376410398924e1_f64) * t105530 * t29160 - F::cast_from(0.26020884564615598386e1_f64) * t97050 * t30740 + F::cast_from(0.17347256376410398924e1_f64) * t26994 * t7637 * t30735 * t1203 - F::cast_from(0.26020884564615598386e1_f64) * t26949 * t7637 * t30735 * t1214 - F::cast_from(0.13170898365871023197e1_f64) * t29207 * t5498;
    t111959
}
