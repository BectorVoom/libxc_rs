//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 937/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk937(t2191: f64, t8592: f64, t2186: f64, t2320: f64, t34902: f64, t7414: f64, t8616: f64, t35584: f64, t35587: f64, t35591: f64, t1550: f64, t2024: f64, t27111: f64, t35567: f64, t35577: f64, t35580: f64, t35593: f64, t40102: f64, t40106: f64, t40110: f64, t40112: f64, t40114: f64) -> f64 {
    let t40116 = t2191 * t8592;
    let t40121 = t2186 * t8592;
    let t40123 = t34902 * t2320;
    let t40124 = 0.24829349937757072982e-4_f64 * t40123;
    let t40125 = t7414 * t8616;
    let t40126 = 0.24829349937757072982e-4_f64 * t40125;
    let t40127 = 0.5854073720911195298e0_f64 * t35584;
    let t40128 = 0.8781110581366792947e0_f64 * t35587;
    let t40129 = 0.2927036860455597649e0_f64 * t35591;
    let t40133 = -0.25538759935978703638e-4_f64 * t40102 + 0.30646511923174444366e-3_f64 * t40106 + 0.76616279807936110914e-4_f64 * t40110 - 0.12769379967989351819e-4_f64 * t40112 - 0.85129199786595678796e-5_f64 * t40114 - 0.25538759935978703638e-4_f64 * t40116 - 0.59590439850616975158e-4_f64 * t35567 - 0.99317399751028291929e-5_f64 * t35577 - 0.19863479950205658386e-4_f64 * t35580 + 0.59590439850616975156e-4_f64 * t40121 + t40124 + t40126 - t40127 + t40128 + t40129 + 0.23948483403727617128e0_f64 * t1550 * t2024 * t27111 + t35593;
    t40133
}
