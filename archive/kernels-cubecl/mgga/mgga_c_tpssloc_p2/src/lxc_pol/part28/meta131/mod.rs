//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta131 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk721;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk722;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk723;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta131<F: Float>(t2906: F, t2932: F, t2786: F, t2789: F, t2796: F, t2839: F, t2847: F, t2853: F, t2856: F, t2861: F, t2863: F, t2881: F, t2886: F, t2889: F, t2898: F, t2900: F, t2905: F, t2907: F, t2925: F, t2930: F, t311: F, t924: F, t933: F, t943: F, t952: F, t300: F, t938: F, t961: F, t2904: F, t951: F, t959: F, t2924: F, t942: F, t2929: F, t2262: F, t338: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2933, t2936) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk721::<F>(t2906, t2932, t2786, t2789, t2796, t2839, t2847, t2853, t2856, t2861, t2863, t2881, t2886, t2889, t2898, t2900, t2905, t2907, t2925, t2930, t311, t924, t933, t943, t952);
        let (t2937, t2939, t2940) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk722::<F>(t2936, t300, t2898, t938);
        let (t2942, t2944, t2946, t2948, t2950, t2952, t2954, t2955) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk723::<F>(t2940, t961, t2904, t2906, t951, t959, t2924, t942, t2929, t2932, t2262, t338);
    (t2933, t2937, t2939, t2940, t2942, t2944, t2946, t2948, t2950, t2952, t2954, t2955)
}
