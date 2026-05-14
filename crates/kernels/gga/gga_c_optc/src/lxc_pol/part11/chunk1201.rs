//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1201/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1201<F: Float>(t57215: F, t57217: F, t57219: F, t57222: F, t57225: F, t57228: F, t57233: F, t57236: F, t57238: F, t57240: F, t57244: F, t1000: F, t13912: F, t1415: F, t24392: F, t2549: F, t3980: F, t43260: F, t5076: F, t52200: F, t52241: F, t52245: F, t57010: F, t57014: F, t57018: F, t57032: F, t57039: F, t57246: F, t57248: F, t7254: F, t914: F, t999: F) -> (F, F) {
    let t58198 = t57215 + t57217 + t57219 - t57222 + t57225 + t57228 - t57233 - t57236 - t57238 - t57240 - t57244;
    let t58226 = -0.10337952573961372198e-1 * t3980 * t52200 * t1415 - 4.0 / 9.0 * t43260 - 4.0 * t999 * t914 * t1000 * t57010 - 2.0 * t13912 * t5076 + 2.0 / 3.0 * t999 * t914 * t2549 * t57018 - t999 * t914 * t1000 * t57014 - t57246 + 8.0 / 9.0 * t52241 + 2.0 / 3.0 * t52245 - 56.0 / 9.0 * t999 * t914 * t7254 * t57032 + 140.0 / 81.0 * t999 * t914 * t24392 * t57039 - t57248;
    (t58198, t58226)
}
