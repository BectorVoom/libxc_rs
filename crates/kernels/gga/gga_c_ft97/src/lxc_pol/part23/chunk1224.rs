//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1224/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1224<F: Float>(t24330: F, t30789: F, t6055: F, t6054: F, t79851: F, t27633: F, t27642: F, t2247: F, t27660: F, t3746: F, t6817: F, t70: F, t123231: F, t18084: F, t218: F, t25057: F, t27500: F, t27658: F, t27717: F, t27729: F, t27733: F, t27736: F, t3766: F, t3817: F, t5025: F, t52588: F, t6018: F, t6057: F, t709: F, t79818: F, t96623: F) -> (F, F, F, F) {
    let t123233 = t24330 * t30789;
    let t123234 = t6055 * t123233;
    let t123236 = t79851 * t6054;
    let t123255 = t27642 * t27633;
    let t123261 = t6817 * t2247 * t70 * t27660 * t3746;
    let t123264 = -0.28374937057613168724e-2 * t123231 - 0.21281202793209876543e-2 * t123234 - 0.12768721675925925926e-1 * t123236 * t6057 - 0.85124811172839506174e-2 * t96623 - 0.2370952259137005195e-1 * t27717 * t79818 + 0.13336606457645654222e-1 * t52588 * t25057 * t218 * t5025 * t709 - 4.0 * t27733 * t27736 - 4.0 * t3766 * t27729 * t3817 - 2.0 * t3766 * t6018 * t18084 + 0.4539989929218106996e-1 * t27500 * t123255 + 0.60548059007656442388e-3 * t27658 * t123261;
    (t123233, t123255, t123261, t123264)
}
