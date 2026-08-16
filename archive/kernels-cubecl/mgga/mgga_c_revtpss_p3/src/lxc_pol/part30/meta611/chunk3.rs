//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2093/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2093<F: Float>(t28011: F, t686: F, t72: F, t7284: F, t7289: F, t14269: F, t25885: F, t25931: F, t27837: F, t28008: F, t7279: F, t7308: F, t94823: F, t94854: F, t94857: F, t94865: F, t94867: F, t98062: F, t98069: F, t98071: F, t98078: F, t98081: F, t98084: F) -> F {
    let t98087 = t28011 * t72 * t686;
    let t98089 = F::cast_from(0.14456046980341999104e-1_f64) * t7284 * t98087;
    let t98091 = F::cast_from(0.25702851531048074406e-1_f64) * t7289 * t98087;
    let t98092 = F::cast_from(0.26020884564615598386e1_f64) * t94823 * t25931 * t98062 + t98069 + t94854 + t98071 + F::cast_from(0.96373646535613327358e-3_f64) * t94857 + F::cast_from(0.8673628188205199462e0_f64) * t27837 * t25885 - F::cast_from(0.65854491829355115987e0_f64) * t7279 * t14269 + F::cast_from(0.86736281882051994623e-1_f64) * t98078 - t98081 - t94865 - F::cast_from(0.8673628188205199462e0_f64) * t28008 * t7308 - t94867 - F::cast_from(0.22849835011101738147e-2_f64) * t98084 + t98089 - t98091;
    t98092
}
