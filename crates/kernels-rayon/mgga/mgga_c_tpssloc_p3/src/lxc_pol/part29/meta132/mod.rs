//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta132 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk760;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk761;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk762;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta132(t2906: f64, t2932: f64, t2786: f64, t2789: f64, t2796: f64, t2839: f64, t2847: f64, t2853: f64, t2856: f64, t2861: f64, t2863: f64, t2881: f64, t2886: f64, t2889: f64, t2898: f64, t2900: f64, t2905: f64, t2907: f64, t2925: f64, t2930: f64, t311: f64, t924: f64, t933: f64, t943: f64, t952: f64, t300: f64, t938: f64, t961: f64, t2904: f64, t951: f64, t959: f64, t2924: f64, t942: f64, t2929: f64, t2262: f64, t338: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2933, t2936) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk760(t2906, t2932, t2786, t2789, t2796, t2839, t2847, t2853, t2856, t2861, t2863, t2881, t2886, t2889, t2898, t2900, t2905, t2907, t2925, t2930, t311, t924, t933, t943, t952);
        let (t2937, t2939, t2940) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk761(t2936, t300, t2898, t938);
        let (t2942, t2944, t2946, t2948, t2950, t2952, t2954, t2955) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk762(t2940, t961, t2904, t2906, t951, t959, t2924, t942, t2929, t2932, t2262, t338);
    (t2933, t2937, t2939, t2940, t2942, t2944, t2946, t2948, t2950, t2952, t2954, t2955)
}
