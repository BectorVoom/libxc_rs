//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1283/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1283(t11698: f64, t6178: f64, t35806: f64, t35809: f64, t35811: f64, t35815: f64, t35820: f64, t35823: f64, t35826: f64, t35829: f64, t35831: f64, t35835: f64, t35838: f64, t35841: f64) -> f64 {
    let t35843 = t6178 * t11698;
    let t35845 = 0.86995919027186744338e-7_f64 * t35806 - 0.23485962392041415794e-4_f64 * t35809 + 0.72904341591961894861e-4_f64 * t35811 + 0.19030357287197100324e-7_f64 * t35815 + 0.4892908831675294957e-7_f64 * t35820 + 0.23485962392041415794e-4_f64 * t35823 + 0.23485962392041415794e-4_f64 * t35826 + 0.11742981196020707897e-4_f64 * t35829 - 0.34197428278281706076e-6_f64 * t35831 - 0.11399142759427235359e-6_f64 * t35835 + 0.41758041133049637282e-5_f64 * t35838 + 0.14678726495025884871e-5_f64 * t35841 + 0.26446011201871186032e-4_f64 * t35843;
    t35845
}
