//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1820/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1820<F: Float>(t189: F, t512: F, t92011: F, t48297: F, t48304: F, t48306: F, t39989: F, t47084: F, t47086: F, t47088: F, t47092: F, t91982: F, t91983: F, t91984: F, t91985: F) -> (F, F, F, F, F) {
    let t92013 = t512 * t92011 * t189;
    let t92014 = F::cast_from(0.4101607543286562663e4_f64) * t48297;
    let t92015 = F::cast_from(0.65061487801810439052e-1_f64) * t48304;
    let t92016 = F::cast_from(0.19263893255070628431e1_f64) * t48306;
    let t92017 = -t91982 - t91983 - t91984 - t91985 + t92013 - t92014 - t47084 + t92015 + t92016 - t39989 - t47086 + t47088 + t47092;
    (t92013, t92014, t92015, t92016, t92017)
}
