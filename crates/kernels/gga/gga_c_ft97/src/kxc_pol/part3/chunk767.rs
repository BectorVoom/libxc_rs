//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 767/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk767<F: Float>(t1564: F, t15917: F, t446: F, t15752: F, t447: F, t15756: F, t3281: F, t432: F, t4462: F, t15604: F, t15606: F, t15609: F, t15612: F, t15617: F, t15621: F, t15628: F, t15888: F, t15891: F, t15894: F, t15897: F, t15899: F, t15904: F, t15907: F, t15910: F, t15915: F) -> (F, F, F, F, F, F) {
    let t15918 = t1564 * t15917;
    let t15919 = t446 * t15918;
    let t15921 = t447 * t15752;
    let t15922 = t446 * t15921;
    let t15924 = t447 * t15756;
    let t15925 = t3281 * t15924;
    let t15927 = t4462 * t432;
    let t15928 = t1564 * t15927;
    let t15929 = t446 * t15928;
    let t15931 = -t15604 + t15606 / F::new(81.0) - t15609 / F::new(27.0) + t15612 / F::new(54.0) + t15617 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t15621 - t15628 / F::new(18.0) - t15888 / F::new(6.0) + t15891 / F::new(18.0) - t15894 / F::new(9.0) - F::new(4.0) / F::new(27.0) * t15897 - t15899 / F::new(27.0) + t15904 / F::new(18.0) + t15907 / F::new(9.0) - t15910 / F::new(27.0) - t15915 / F::new(9.0) - t15919 / F::new(9.0) - t15922 / F::new(3.0) + F::new(4.0) / F::new(9.0) * t15925 + t15929 / F::new(18.0);
    (t15919, t15922, t15925, t15927, t15929, t15931)
}
