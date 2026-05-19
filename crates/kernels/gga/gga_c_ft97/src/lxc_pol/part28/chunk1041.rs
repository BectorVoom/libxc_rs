//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1041/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1041<F: Float>(t136282: F, t136365: F, t136369: F, t136434: F, t136604: F, t136814: F, t145160: F, t145163: F, t145168: F, t145171: F, t145188: F, t145192: F, t1669: F, t22597: F, t25698: F, t25703: F, t25788: F, t25826: F, t25835: F, t32241: F, t9: F, t92809: F) -> F {
    let t145195 = -F::cast_from(0.10338048737805743097e-3_f64) * t136604 * t25826 - F::cast_from(0.78259321553885081522e-2_f64) * t145160 * t145163 + F::cast_from(0.65216101294904234602e-2_f64) * t145160 * t145168 + F::cast_from(0.78259321553885081522e-2_f64) * t136434 * t32241 * t145171 * t25698 - F::cast_from(0.11738898233082762228e-1_f64) * t136282 * t32241 * t145171 * t25703 - F::cast_from(0.10338048737805743097e-3_f64) * t136604 * t25835 - F::cast_from(0.45967398033333333333e0_f64) * t1669 * t92809 * t9 * t25788 - F::cast_from(0.13200366700519885118e-5_f64) * t136365 + F::cast_from(0.29693535778629056444e-3_f64) * t136369 + F::cast_from(0.25845121844514357744e-4_f64) * t136814 * t145188 + F::cast_from(0.51690243689028715488e-5_f64) * t22597 * t145192;
    t145195
}
