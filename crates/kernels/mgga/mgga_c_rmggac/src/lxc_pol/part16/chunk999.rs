//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 999/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk999<F: Float>(t2265: F, t6624: F, t10367: F, t275: F, t47874: F, t47876: F, t47881: F, t47883: F, t47885: F, t47887: F, t47889: F, t47891: F, t47898: F, t47903: F, t47908: F, t47913: F, t47918: F, t47923: F, t47931: F, t4985: F, t9352: F) -> (F,) {
    let t49736 = t6624 * t2265;
    let t49738 = t275 * t10367;
    let t49747 = -0.1702583995731913576e-4 * t47874 - 0.638468998399467591e-4 * t47876 + 0.85129199786595678799e-5 * t47881 - 0.85129199786595678799e-5 * t47883 - 0.20455996240684006298e-1 * t47885 + 0.2727466165424534173e-1 * t47887 + 0.13637330827122670865e-1 * t47889 + 0.40911992481368012596e-1 * t47891 + 0.11974241701863808564e0 * t4985 * t9352 - 0.2363e1 * t49736 + 2.0 * t49738 + 0.638468998399467591e-4 * t47898 - 0.1276937996798935182e-3 * t47903 + 0.1915406995198402773e-3 * t47908 + 0.638468998399467591e-4 * t47913 - 0.638468998399467591e-4 * t47918 + 0.5107751987195740728e-4 * t47923 - 0.35922725105591425692e0 * t47931;
    (t49747,)
}
