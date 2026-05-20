//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1678/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1678<F: Float>(t1248: F, t12629: F, t12646: F, t12712: F, t12717: F, t12734: F, t12741: F, t12751: F, t1287: F, t12975: F, t12987: F, t13144: F, t13150: F, t3302: F, t3746: F, t3759: F, t3760: F, t3767: F, t3769: F, t3782: F, t3783: F, t3784: F, t45609: F, t45648: F, t45734: F, t45764: F, t45769: F, t45779: F, t45786: F, t45787: F, t45796: F) -> F {
    let t45800 = -F::cast_from(0.39512695097613069592e1_f64) * t3782 * t45734 * t3783 - F::cast_from(0.39512695097613069592e1_f64) * t45764 * t3784 + F::cast_from(0.79025390195226139183e1_f64) * t3746 * t12741 + F::cast_from(0.15805078039045227836e2_f64) * t45769 * t13150 - F::cast_from(0.79025390195226139183e1_f64) * t12975 * t3760 + F::cast_from(0.39512695097613069591e1_f64) * t3767 * t45648 * t3769 + F::cast_from(0.26341796731742046395e1_f64) * t3746 * t12734 - F::cast_from(0.15805078039045227836e2_f64) * t45779 * t13144 - F::cast_from(0.15805078039045227836e2_f64) * t12987 * t3759 * t12629 + F::cast_from(0.92196288561097162379e1_f64) * t45786 * t45609 * t45787 + F::cast_from(0.15805078039045227836e2_f64) * t12717 * t12646 * t1248 * t1287 - F::cast_from(0.15805078039045227836e2_f64) * t12751 * t12712 * t3302 * t45796;
    t45800
}
