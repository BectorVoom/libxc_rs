//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta156 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk793;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk794;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk795;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk796;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta156(t3426: f64, t461: f64, t221: f64, t456: f64, t1176: f64, t135: f64, t1179: f64, t1174: f64, t1186: f64, t1089: f64, t405: f64, t974: f64, t3242: f64, t2244: f64, t337: f64, t51: f64, t1887: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3428, t3430, t3431) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk793(t3426, t461, t221, t456, t1176, t135);
        let (t3432, t3433, t3435, t3436, t3439) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk794(t1179, t3431, t1174, t1186, t135, t1089, t405);
        let t3440 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk795(t3439, t974);
        let (t3442, t3443, t3447) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk796(t3242, t461, t2244, t3440, t337, t51, t1887);
    (t3428, t3430, t3431, t3432, t3433, t3435, t3436, t3439, t3440, t3442, t3443, t3447)
}
