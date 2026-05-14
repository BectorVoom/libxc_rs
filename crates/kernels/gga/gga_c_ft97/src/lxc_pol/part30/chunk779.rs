//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 779/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk779<F: Float>(t290: F, t5009: F, t35462: F, t1201: F, t1209: F, t19039: F, t28558: F, t28603: F, t28652: F, t28660: F, t28677: F, t28680: F, t292: F, t31462: F, t31535: F, t33903: F, t33925: F, t33947: F, t33948: F, t35358: F, t35402: F, t35406: F, t35467: F, t35872: F, t35879: F, t35887: F, t35890: F, t35902: F, t4113: F, t5265: F, t7003: F, t7590: F, t7607: F) -> (F, F) {
    let t35908 = t290 * t5009;
    let t35909 = t35908 * t35462;
    let t35914 = -0.21340587982578139066e0 * t7607 * t35467 + 0.18125821328051150223e0 * t28680 * t35872 + 0.26675734978222673832e-1 * t33948 * t35406 + 0.18125821328051150223e0 * t28652 * t35879 - 0.18125821328051150223e0 * t28660 * t35872 - 0.18125821328051150223e0 * t28677 * t35879 + 0.22914129771549286116e-1 * t31535 * t35887 + 0.20527106943485609994e0 * t19039 * t35890 - 0.27369475924647479993e1 * t292 * t35402 + 0.54738951849294959985e1 * t1201 * t35402 + 0.30209702213418583705e-1 * t28558 * t35358 - 0.30209702213418583705e-1 * t28603 * t35358 - 0.22653425206514361674e0 * t31462 * t35902 - 0.10263553471742804997e0 * t5265 * t7590 * t1209 - t33903 + t33925 + t33947 - 0.45828259543098572232e-1 * t7003 * t35909 + 0.15276086514366190744e-1 * t4113 * t35909;
    (t35908, t35914)
}
