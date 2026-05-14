//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1021/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1021<F: Float>(t5674: F, t771: F, t310: F, t5999: F, t5989: F, t751: F, t2021: F, t296: F, t2030: F, t5913: F, t2970: F, t2177: F, t91: F, t204: F, t3981: F, t824: F) -> (F, F, F, F, F, F, F, F) {
    let t18236 = t771 * t5674;
    let t18258 = 1.0 / t5999 / t310;
    let t18284 = t751 * t5989;
    let t18290 = 1.0 / t2021 / t296;
    let t18331 = t2030 * t5913;
    let t18332 = t2970 * t18331;
    let t18406 = t2177 * t2177;
    let t18408 = 1.0 / t91 / t18406;
    let t18427 = t204 * t3981 * t824;
    (t18236, t18258, t18284, t18290, t18331, t18332, t18408, t18427)
}
