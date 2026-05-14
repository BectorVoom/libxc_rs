//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 932/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk932<F: Float>(t10252: F, t38965: F, t42788: F, t42794: F, t45055: F, t45060: F, t45062: F, t45064: F, t45069: F, t45074: F, t45080: F, t45087: F, t45089: F, t45091: F, t45094: F, t45099: F, t45104: F, t45109: F, t5016: F) -> (F,) {
    let t48342 = -0.5107751987195740728e-4 * t45055 + 0.1915406995198402773e-3 * t45060 - 0.638468998399467591e-4 * t45062 - 0.5107751987195740728e-4 * t45064 + 0.85129199786595678799e-5 * t45069 - 0.2553875993597870364e-4 * t45074 + 0.2553875993597870364e-4 * t45080 - 0.11974241701863808564e0 * t5016 * t10252 + t42788 + 0.20455996240684006298e-1 * t45087 - 0.23836175940246790064e-3 * t45089 - 0.13242319966803772257e-3 * t38965 + t42794 - 0.85129199786595678799e-5 * t45091 - 0.85129199786595678799e-5 * t45094 + 0.1702583995731913576e-4 * t45099 + 0.1702583995731913576e-4 * t45104 + 0.85129199786595678799e-5 * t45109;
    (t48342,)
}
