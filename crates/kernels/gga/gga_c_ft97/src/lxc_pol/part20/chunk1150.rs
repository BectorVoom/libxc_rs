//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1150/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1150<F: Float>(t1456: F, t2492: F, t28357: F, t8392: F, t1882: F, t28322: F, t28326: F, t28398: F, t10153: F, t1175: F, t13702: F, t13848: F, t14127: F, t14259: F, t1901: F, t24460: F, t24465: F, t24668: F, t2469: F, t24789: F, t2574: F, t27841: F, t27889: F, t28276: F, t446: F, t6861: F, t713: F, t729: F, t762: F, t773: F, t97637: F, t97639: F) -> (F,) {
    let t110478 = t2492 * t1456;
    let t110489 = 2.0 / 27.0 * t8392 * t28357;
    let t110496 = 4.0 / 9.0 * t1882 * t28322;
    let t110498 = 4.0 / 9.0 * t1882 * t28326;
    let t110503 = 2.0 / 27.0 * t8392 * t28398;
    let t110504 = 4.0 / 3.0 * t446 * t2574 * t773 * t27841 + t446 * t729 * t10153 * t6861 / 3.0 + 2.0 / 3.0 * t446 * t729 * t2469 * t28276 + 4.0 / 3.0 * t446 * t2574 * t1175 * t24460 + 2.0 / 3.0 * t446 * t2574 * t1175 * t24465 - 4.0 / 9.0 * t1901 * t110478 * t13702 - 2.0 / 3.0 * t1901 * t14127 * t24668 * t14259 + t97637 / 9.0 + 2.0 / 9.0 * t97639 - t110489 + 2.0 / 3.0 * t446 * t729 * t762 * t27889 * t713 - t110496 - t110498 - 2.0 / 9.0 * t1901 * t24789 * t13848 - t110503;
    (t110504,)
}
