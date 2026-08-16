//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta112 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk616;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk617;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk618;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta112(t135: f64, t999: f64, t973: f64, t2250: f64, t998: f64, t974: f64, t2770: f64, t2978: f64, t2244: f64, t2775: f64, t976: f64, t1005: f64, t1036: f64, t221: f64, t2965: f64, t339: f64, t964: f64, t995: f64, t1000: f64, t1020: f64, t1025: f64, t1046: f64, t2955: f64, t2960: f64, t3109: f64, t3114: f64, t3117: f64, t3123: f64, t3130: f64, t3134: f64, t350: f64, t3106: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3139, t3140, t3142, t3143, t3146, t3147, t3148, t3151, t3152, t3153, t3156) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk616(t135, t999, t973, t2250, t998, t974, t2770, t2978, t2244, t2775, t976, t1005, t1036);
        let (t3158, t3165) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk617(t221, t2965, t339, t964, t995, t1000, t1020, t1025, t1046, t2955, t2960, t3109, t3114, t3117, t3123, t3130, t3134, t3140, t3143, t3148, t3153, t3156, t350, t973);
        let t3166 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk618(t3106, t3165);
    (t3139, t3142, t3143, t3146, t3147, t3148, t3151, t3152, t3153, t3158, t3166)
}
