//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 731/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk731<F: Float>(t10851: F, t10872: F, t10874: F, t1033: F, t3392: F, t10876: F, t10879: F, t10500: F, t954: F, t1809: F, t1620: F, t2615: F, t3415: F, t12744: F, t12746: F, t12750: F, t12754: F, t12756: F, t12758: F, t5359: F, t5948: F, t5952: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12759 = 16.0 / 45.0 * t10851;
    let t12760 = 4.0 / 15.0 * t10872;
    let t12761 = 8.0 / 15.0 * t10874;
    let t12763 = 4.0 / 5.0 * t1033 * t3392;
    let t12764 = 16.0 / 15.0 * t10876;
    let t12765 = 8.0 / 15.0 * t10879;
    let t12766 = t10500 * t954;
    let t12767 = t1809 * t12766;
    let t12769 = 8.0 / 15.0 * t1620 * t12767;
    let t12771 = 8.0 / 15.0 * t2615 * t3415;
    let t12772 = t5359 - t12744 + t12746 - t12750 + t12754 + t12756 + t5948 + t5952 + t12758 + t12759 - t12760 - t12761 + t12763 + t12764 + t12765 + t12769 - t12771;
    (t12759, t12760, t12761, t12763, t12764, t12765, t12766, t12767, t12769, t12771, t12772)
}
