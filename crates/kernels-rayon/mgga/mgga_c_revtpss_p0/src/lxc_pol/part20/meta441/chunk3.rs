//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1678/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1678(t1248: f64, t12629: f64, t12646: f64, t12712: f64, t12717: f64, t12734: f64, t12741: f64, t12751: f64, t1287: f64, t12975: f64, t12987: f64, t13144: f64, t13150: f64, t3302: f64, t3746: f64, t3759: f64, t3760: f64, t3767: f64, t3769: f64, t3782: f64, t3783: f64, t3784: f64, t45609: f64, t45648: f64, t45734: f64, t45764: f64, t45769: f64, t45779: f64, t45786: f64, t45787: f64, t45796: f64) -> f64 {
    let t45800 = -0.39512695097613069592e1_f64 * t3782 * t45734 * t3783 - 0.39512695097613069592e1_f64 * t45764 * t3784 + 0.79025390195226139183e1_f64 * t3746 * t12741 + 0.15805078039045227836e2_f64 * t45769 * t13150 - 0.79025390195226139183e1_f64 * t12975 * t3760 + 0.39512695097613069591e1_f64 * t3767 * t45648 * t3769 + 0.26341796731742046395e1_f64 * t3746 * t12734 - 0.15805078039045227836e2_f64 * t45779 * t13144 - 0.15805078039045227836e2_f64 * t12987 * t3759 * t12629 + 0.92196288561097162379e1_f64 * t45786 * t45609 * t45787 + 0.15805078039045227836e2_f64 * t12717 * t12646 * t1248 * t1287 - 0.15805078039045227836e2_f64 * t12751 * t12712 * t3302 * t45796;
    t45800
}
