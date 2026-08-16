//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1656;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1657;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1658;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1659;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1660;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1661;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta456(t225: f64, t7085: f64, t10110: f64, t2053: f64, t2719: f64, t23251: f64, t23261: f64, t7106: f64, t865: f64, t2718: f64, t2742: f64, t10049: f64, t2054: f64, t23243: f64, t23249: f64, t23254: f64, t23259: f64, t23266: f64, t23274: f64, t2597: f64, t2713: f64, t2743: f64, t7087: f64, t7092: f64, t7107: f64, t855: f64, t866: f64, t9590: f64, t9593: f64, t24300: f64, t870: f64, t2752: f64, t7109: f64, t10143: f64, t2056: f64, t1877: f64, t2057: f64, t2249: f64, t22951: f64, t22961: f64, t22964: f64, t22968: f64, t23296: f64, t23299: f64, t23302: f64, t24191: f64, t25: f64, t2522: f64, t4314: f64, t606: f64, t6542: f64, t6671: f64, t7110: f64, t7114: f64, t13487: f64, t193: f64, t202: f64, t2379: f64, t2553: f64, t2745: f64, t2749: f64, t776: f64, t868: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24305, t24314, t24318, t24321, t24325, t24330, t24333) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1656(t225, t7085, t10110, t2053, t2719, t23251, t23261, t7106, t865, t2718, t2742, t10049, t2054, t23243, t23249, t23254, t23259, t23266, t23274, t2597, t2713, t2743, t7087, t7092, t7107, t855, t866, t9590, t9593);
        let (t24334, t24335) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1657(t24300, t24333, t870);
        let t24339 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1658(t2752, t7109);
        let t24344 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1659(t10143, t2056);
        let t24355 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1660(t1877, t2057, t2249, t22951, t22961, t22964, t22968, t23296, t23299, t23302, t24191, t24335, t24339, t24344, t25, t2522, t4314, t606, t6542, t6671, t7110, t7114);
        let t24379 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1661(t13487, t1877, t193, t202, t2057, t2379, t24334, t24339, t24344, t2522, t2553, t2745, t2749, t4314, t7110, t7114, t776, t868, t870);
    (t24305, t24314, t24318, t24321, t24325, t24330, t24334, t24335, t24339, t24344, t24355, t24379)
}
