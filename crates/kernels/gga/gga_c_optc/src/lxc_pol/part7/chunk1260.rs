//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1260/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1260<F: Float>(t1220: F, t2367: F, t8435: F, t4281: F, t9142: F, t9240: F, t11885: F, t9243: F, t8430: F, t3274: F, t9233: F, t2839: F, t2905: F, t26160: F, t26163: F, t26168: F, t26170: F, t26173: F, t27346: F, t3245: F, t4290: F) -> (F,) {
    let t27843 = t1220 * t2367 * t8435;
    let t27846 = t4281 * t9142 * t9240;
    let t27849 = t4281 * t11885 * t9243;
    let t27856 = t1220 * t2367 * t8430;
    let t27858 = t3274 * t9233;
    let t27860 = t2905 * t2839;
    let t27862 = -t26160 + t26163 + 2.0 / 9.0 * t27843 + t26168 + t26170 - 4.0 / 3.0 * t27846 + 8.0 / 9.0 * t27849 - t26173 + 6.0 * t4281 * t3245 * t4290 * t27346 + 56.0 / 81.0 * t27856 + 8.0 / 9.0 * t27858 - 2.0 / 3.0 * t27860;
    (t27862,)
}
