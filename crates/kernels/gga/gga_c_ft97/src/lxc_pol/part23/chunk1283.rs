//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1283/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1283<F: Float>(t31160: F, t8392: F, t1882: F, t31143: F, t110629: F, t110660: F, t111068: F, t11593: F, t1175: F, t13885: F, t18206: F, t18617: F, t18681: F, t1901: F, t24599: F, t24737: F, t24747: F, t24793: F, t2574: F, t2599: F, t2606: F, t27878: F, t28123: F, t28140: F, t28284: F, t3746: F, t3837: F, t3842: F, t3977: F, t446: F, t4965: F, t4969: F, t6074: F, t729: F, t97964: F, t97966: F, t9803: F) -> (F,) {
    let t124714 = t8392 * t31160;
    let t124720 = t1882 * t31143;
    let t124737 = 8.0 / 27.0 * t97964 + 4.0 / 27.0 * t97966 + 2.0 / 27.0 * t1901 * t9803 * t24747 * t4965 + 8.0 * t1901 * t110660 * t6074 * t18206 + 2.0 * t1901 * t28140 * t24737 * t18617 - 4.0 * t1901 * t28140 * t28123 * t3837 - 4.0 / 3.0 * t1901 * t13885 * t110629 * t3842 - 2.0 / 27.0 * t124714 - 2.0 / 9.0 * t1901 * t2606 * t24599 * t4969 - 2.0 / 9.0 * t124720 - 2.0 / 9.0 * t1901 * t24793 * t18681 - 4.0 / 9.0 * t11593 * t2599 * t28123 * t3746 + 4.0 / 3.0 * t446 * t2574 * t1175 * t27878 + 2.0 / 3.0 * t446 * t729 * t3977 * t28284 + t111068;
    (t124737,)
}
