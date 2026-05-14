//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 833/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk833<F: Float>(t40123: F, t7414: F, t8616: F, t35584: F, t35587: F, t35591: F, t1550: F, t2024: F, t27111: F, t35567: F, t35577: F, t35580: F, t35593: F, t40102: F, t40106: F, t40110: F, t40112: F, t40114: F, t40116: F, t40121: F) -> (F,) {
    let t40124 = 0.24829349937757072982e-4 * t40123;
    let t40125 = t7414 * t8616;
    let t40126 = 0.24829349937757072982e-4 * t40125;
    let t40127 = 0.5854073720911195298e0 * t35584;
    let t40128 = 0.8781110581366792947e0 * t35587;
    let t40129 = 0.2927036860455597649e0 * t35591;
    let t40133 = -0.25538759935978703638e-4 * t40102 + 0.30646511923174444366e-3 * t40106 + 0.76616279807936110914e-4 * t40110 - 0.12769379967989351819e-4 * t40112 - 0.85129199786595678796e-5 * t40114 - 0.25538759935978703638e-4 * t40116 - 0.59590439850616975158e-4 * t35567 - 0.99317399751028291929e-5 * t35577 - 0.19863479950205658386e-4 * t35580 + 0.59590439850616975156e-4 * t40121 + t40124 + t40126 - t40127 + t40128 + t40129 + 0.23948483403727617128e0 * t1550 * t2024 * t27111 + t35593;
    (t40133,)
}
