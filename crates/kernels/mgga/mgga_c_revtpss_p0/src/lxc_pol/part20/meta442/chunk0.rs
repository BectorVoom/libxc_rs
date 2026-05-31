//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1687/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1687<F: Float>(t12: F, t14: F, t27: F, t10285: F, t596: F, t2231: F, t2237: F, t10293: F, t592: F, t25: F, t40649: F, t45927: F, t45929: F, t45931: F, t45933: F, t45935: F, t45937: F, t45939: F, t45941: F) -> F {
    let t45944 = F::cast_from(360.0_f64) * t12 * t14 * t27;
    let t45945 = t10285 * t596;
    let t45946 = F::cast_from(2880.0_f64) * t45945;
    let t45947 = t2231 * t2237;
    let t45948 = F::cast_from(7560.0_f64) * t45947;
    let t45949 = t592 * t10293;
    let t45950 = F::cast_from(8064.0_f64) * t45949;
    let t45952 = F::cast_from(3024.0_f64) * t25 * t40649;
    let t45953 = t45927 - t45929 + t45931 + t45933 - t45935 + t45937 - t45939 + t45941 + t45944 - t45946 + t45948 - t45950 + t45952;
    t45953
}
