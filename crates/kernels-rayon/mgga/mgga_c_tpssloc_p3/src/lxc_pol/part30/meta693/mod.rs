//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta693 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2208;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2209;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2210;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2211;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2212;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2213;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta693(t1888: f64, t25045: f64, t86873: f64, t214: f64, t5631: f64, t1880: f64, t6572: f64, t22986: f64, t23270: f64, t5657: f64, t776: f64, t857: f64, t258: f64, t5527: f64, t87642: f64, t6552: f64, t7479: f64, t87782: f64, t13053: f64, t13065: f64, t13463: f64, t17049: f64, t1911: f64, t25348: f64, t2597: f64, t2718: f64, t28307: f64, t28317: f64, t4273: f64, t7517: f64, t7538: f64, t855: f64, t86844: f64, t86869: f64, t86887: f64, t86896: f64, t92383: f64, t98117: f64, t98122: f64, t2717: f64, t5636: f64, t225: f64, t28437: f64, t5544: f64, t25038: f64, t865: f64, t23237: f64, t28294: f64, t28267: f64, t82159: f64, t25054: f64, t6555: f64, t25216: f64, t25224: f64, t25040: f64, t17052: f64, t6663: f64, t82070: f64, t82082: f64, t86929: f64, t92406: f64, t81651: f64, t82074: f64, t25044: f64, t4300: f64, t23035: f64, t28298: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98125, t98133, t98135, t98148) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2208(t1888, t25045, t86873, t214, t5631, t1880, t6572, t22986, t23270, t5657, t776, t857);
        let t98160 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2209(t23270, t258, t5527, t776, t87642, t6552, t7479, t87782, t13053, t13065, t13463, t17049, t1911, t25348, t2597, t2718, t28307, t28317, t4273, t7517, t7538, t855, t86844, t86869, t86887, t86896, t92383, t98117, t98122, t98125, t98135, t98148);
        let (t98164, t98166, t98172, t98181) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2210(t2717, t5636, t22986, t23270, t776, t225, t28437, t258, t5544, t25038, t1888, t5657, t865);
        let (t98189, t98192, t98196, t98199, t98202) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2211(t1880, t23237, t28294, t22986, t28267, t82159, t25054, t86873, t6552, t6555, t98133, t25216, t25224);
        let t98208 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2212(t25038, t25040, t86873, t17052, t6663, t82070, t82082, t86929, t92406, t98189, t98192, t98196, t98199, t98202);
        let (t98213, t98222, t98227, t98234) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2213(t28267, t81651, t82074, t1888, t23270, t25044, t4300, t5527, t857, t25038, t865, t23035, t23237, t28298);
    (t98160, t98164, t98166, t98172, t98181, t98208, t98213, t98222, t98227, t98234)
}
