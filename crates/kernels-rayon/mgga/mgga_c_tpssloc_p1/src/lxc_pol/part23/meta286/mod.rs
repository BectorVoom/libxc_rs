//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta286 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk986;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk987;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk988;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk989;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk990;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk991;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta286(t21013: f64, t218: f64, t1528: f64, t17052: f64, t17090: f64, t17092: f64, t21034: f64, t21036: f64, t21038: f64, t21050: f64, t21054: f64, t21061: f64, t259: f64, t4147: f64, t4268: f64, t5637: f64, t5658: f64, t855: f64, t10143: f64, t1484: f64, t16625: f64, t193: f64, t202: f64, t20777: f64, t20778: f64, t20800: f64, t20815: f64, t2522: f64, t4310: f64, t5544: f64, t766: f64, t870: f64, t9820: f64, t9824: f64, t9876: f64, t9884: f64, t9887: f64, t9890: f64, t20818: f64, t20820: f64, t20822: f64, t20823: f64, t20824: f64, t20827: f64, t20829: f64, t20830: f64, t20831: f64, t9853: f64, t9859: f64, t9894: f64, t9907: f64, t9921: f64, t20752: f64, t20772: f64, t1580: f64, t5774: f64, t2929: f64, t951: f64, t959: f64, t10523: f64, t2932: f64, t1589: f64, t17934: f64, t10629: f64, t10632: f64, t4483: f64, t5808: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21064, t21066) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk986(t21013, t218, t1528, t17052, t17090, t17092, t21034, t21036, t21038, t21050, t21054, t21061, t259, t4147, t4268, t5637, t5658, t855);
        let t21073 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk987(t10143, t1484, t16625, t193, t202, t20777, t20778, t20800, t20815, t21066, t2522, t4310, t5544, t766, t870, t9820, t9824, t9876, t9884, t9887, t9890);
        let t21074 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk988(t20818, t20820, t20822, t20823, t20824, t20827, t20829, t20830, t20831, t9853, t9859, t9894, t9907, t9921);
        let t21076 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk989(t20752, t20772, t21073, t21074);
        let t21089 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk990(t1580, t5774);
        let (t21091, t21093, t21095, t21097, t21099, t21101, t21103, t21105) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk991(t21089, t2929, t951, t959, t10523, t2932, t1589, t17934, t10629, t10632, t4483, t5808);
    (t21064, t21066, t21076, t21089, t21091, t21093, t21095, t21097, t21099, t21101, t21103, t21105)
}
