//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1073/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1073(t10252: f64, t38965: f64, t42788: f64, t42794: f64, t45055: f64, t45060: f64, t45062: f64, t45064: f64, t45069: f64, t45074: f64, t45080: f64, t45087: f64, t45089: f64, t45091: f64, t45094: f64, t45099: f64, t45104: f64, t45109: f64, t5016: f64) -> f64 {
    let t48342 = -0.5107751987195740728e-4_f64 * t45055 + 0.1915406995198402773e-3_f64 * t45060 - 0.638468998399467591e-4_f64 * t45062 - 0.5107751987195740728e-4_f64 * t45064 + 0.85129199786595678799e-5_f64 * t45069 - 0.2553875993597870364e-4_f64 * t45074 + 0.2553875993597870364e-4_f64 * t45080 - 0.11974241701863808564e0_f64 * t5016 * t10252 + t42788 + 0.20455996240684006298e-1_f64 * t45087 - 0.23836175940246790064e-3_f64 * t45089 - 0.13242319966803772257e-3_f64 * t38965 + t42794 - 0.85129199786595678799e-5_f64 * t45091 - 0.85129199786595678799e-5_f64 * t45094 + 0.1702583995731913576e-4_f64 * t45099 + 0.1702583995731913576e-4_f64 * t45104 + 0.85129199786595678799e-5_f64 * t45109;
    t48342
}
