//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2093/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2093(t28011: f64, t686: f64, t72: f64, t7284: f64, t7289: f64, t14269: f64, t25885: f64, t25931: f64, t27837: f64, t28008: f64, t7279: f64, t7308: f64, t94823: f64, t94854: f64, t94857: f64, t94865: f64, t94867: f64, t98062: f64, t98069: f64, t98071: f64, t98078: f64, t98081: f64, t98084: f64) -> f64 {
    let t98087 = t28011 * t72 * t686;
    let t98089 = 0.14456046980341999104e-1_f64 * t7284 * t98087;
    let t98091 = 0.25702851531048074406e-1_f64 * t7289 * t98087;
    let t98092 = 0.26020884564615598386e1_f64 * t94823 * t25931 * t98062 + t98069 + t94854 + t98071 + 0.96373646535613327358e-3_f64 * t94857 + 0.8673628188205199462e0_f64 * t27837 * t25885 - 0.65854491829355115987e0_f64 * t7279 * t14269 + 0.86736281882051994623e-1_f64 * t98078 - t98081 - t94865 - 0.8673628188205199462e0_f64 * t28008 * t7308 - t94867 - 0.22849835011101738147e-2_f64 * t98084 + t98089 - t98091;
    t98092
}
