//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1284/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1284<F: Float>(t11698: F, t6178: F, t35806: F, t35809: F, t35811: F, t35815: F, t35820: F, t35823: F, t35826: F, t35829: F, t35831: F, t35835: F, t35838: F, t35841: F) -> F {
    let t35843 = t6178 * t11698;
    let t35845 = F::new(0.86995919027186744338e-7) * t35806 - F::new(0.23485962392041415794e-4) * t35809 + F::new(0.72904341591961894861e-4) * t35811 + F::new(0.19030357287197100324e-7) * t35815 + F::new(0.4892908831675294957e-7) * t35820 + F::new(0.23485962392041415794e-4) * t35823 + F::new(0.23485962392041415794e-4) * t35826 + F::new(0.11742981196020707897e-4) * t35829 - F::new(0.34197428278281706076e-6) * t35831 - F::new(0.11399142759427235359e-6) * t35835 + F::new(0.41758041133049637282e-5) * t35838 + F::new(0.14678726495025884871e-5) * t35841 + F::new(0.26446011201871186032e-4) * t35843;
    t35845
}
