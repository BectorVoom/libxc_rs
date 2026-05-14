//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 578/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk578<F: Float>(t486: F, t100: F, t1853: F, t492: F, t83: F, t1570: F, t487: F, t8211: F, t1909: F, t1843: F, t376: F, t89: F, t7822: F, t7775: F, t7778: F, t7748: F, t7758: F, t7768: F, t7791: F, t7796: F, t7809: F, t7813: F, t7817: F, t7827: F, t7831: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8416 = t486 * t486;
    let t8417 = 1.0 / t8416;
    let t8418 = t100 * t8417;
    let t8419 = t1853 * t492;
    let t8420 = t8418 * t8419;
    let t8421 = t83 * t8420;
    let t8424 = t487 * t1570;
    let t8425 = t8424 * t8211;
    let t8426 = t1909 * t8425;
    let t8430 = t89 * t376 * t1843;
    let t8437 = 2.0 / 9.0 * t7822;
    let t8443 = 4.0 / 27.0 * t7775;
    let t8444 = t7778 / 9.0;
    let t8445 = 2.0 / 3.0 * t7791 + 2.0 / 9.0 * t7796 - 2.0 / 9.0 * t7809 + t7813 / 3.0 + t7817 / 3.0 - t8437 - 2.0 / 3.0 * t7827 - 2.0 / 3.0 * t7831 - t7748 / 9.0 + 2.0 * t7758 - 10.0 / 81.0 * t7768 - t8443 + t8444;
    (t8416, t8417, t8418, t8419, t8420, t8421, t8425, t8426, t8430, t8445)
}
