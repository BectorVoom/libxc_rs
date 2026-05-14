//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 783/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk783<F: Float>(t12998: F, t12974: F, t12983: F, t3661: F, t26: F, t1186: F, t12868: F, t306: F, t315: F, t12970: F, t12959: F, t12962: F, t12965: F, t12967: F, t12971: F, t12985: F, t12989: F, t12993: F, t12995: F) -> (F, F, F, F, F) {
    let t12999 = 0.36514074074074074075e0 * t12998;
    let t13000 = 0.93011851851851851854e0 * t12974;
    let t13001 = t3661 * t12983;
    let t13002 = t26 * t13001;
    let t13004 = t1186 * t12868;
    let t13005 = t26 * t13004;
    let t13009 = 1.0 / t306 / t315 / 4.0;
    let t13010 = t13009 * t12970;
    let t13014 = -0.17938e1 * t12959 + 0.16431333333333333333e0 * t12962 - 0.49293999999999999999e0 * t12965 - 0.32862666666666666666e0 * t12967 - 0.76790625e-1 * t12971 + 0.1898925e1 * t12993 + 0.3071625e0 * t12995 - t12999 - t13000 - 0.82156666666666666668e-1 * t13002 + 0.49293999999999999999e0 * t13005 + 0.142419375e1 * t13010 - 0.59793333333333333333e0 * t12985 + 0.17938e1 * t12989;
    (t13002, t13005, t13009, t13010, t13014)
}
