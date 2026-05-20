//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2259/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2259<F: Float>(t105420: F, t111987: F, t111991: F, t1214: F, t1269: F, t21333: F, t2144: F, t2152: F, t27011: F, t27020: F, t27025: F, t29175: F, t29193: F, t29196: F, t29264: F, t29275: F, t29304: F, t30752: F, t30849: F, t30882: F, t30886: F, t30906: F, t5215: F, t5237: F, t5246: F, t6588: F, t6745: F, t7636: F, t7637: F, t7643: F, t7652: F, t8190: F, t8205: F, t96927: F, t96929: F, t96953: F, t96954: F, t96986: F, t97308: F) -> F {
    let t112645 = -F::cast_from(0.17347256376410398924e1_f64) * t8205 * t29193 * t29196 - F::cast_from(0.8673628188205199462e0_f64) * t27025 * t30752 + F::cast_from(0.13170898365871023197e1_f64) * t29304 * t5237 - F::cast_from(0.34694512752820797848e1_f64) * t96927 * t30849 * t96929 - F::cast_from(0.52041769129231196772e1_f64) * t105420 * t29264 - F::cast_from(0.34694512752820797848e1_f64) * t7643 * t7652 * t30886 * t1214 - F::cast_from(0.17347256376410398924e1_f64) * t7636 * t7637 * t8190 * t5215 + F::cast_from(0.34694512752820797848e1_f64) * t96986 * t30849 * t111987 - F::cast_from(0.17347256376410398924e1_f64) * t97308 * t30849 * t111991 - F::cast_from(0.65854491829355115987e0_f64) * t27011 * t6588 + F::cast_from(0.34694512752820797848e1_f64) * t96953 * t30906 * t96954 - F::cast_from(0.8673628188205199462e0_f64) * t30882 * t1269 * t2152 - F::cast_from(0.65854491829355115987e0_f64) * t27020 * t6745 - F::cast_from(0.13170898365871023197e1_f64) * t29304 * t5246 + F::cast_from(0.65854491829355115987e0_f64) * t21333 * t2144 - F::cast_from(0.17347256376410398924e1_f64) * t29275 * t29175;
    t112645
}
