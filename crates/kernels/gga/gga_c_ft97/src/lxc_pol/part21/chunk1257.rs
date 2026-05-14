//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1257/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1257<F: Float>(t12277: F, t6718: F, t23478: F, t4805: F, t4724: F, t95021: F, t5968: F, t9439: F, t30127: F, t40591: F, t1053: F, t2179: F, t27191: F, t30130: F, t9276: F, t104627: F, t104632: F, t1349: F, t1362: F, t2: F, t26: F, t26533: F, t26538: F, t26791: F, t28: F, t4: F, t78726: F) -> (F, F, F, F, F, F, F, F, F) {
    let t119443 = t12277 * t6718;
    let t119445 = t23478 * t4805;
    let t119447 = t95021 * t4724;
    let t119450 = t9439 * t5968 * t4724;
    let t119452 = t40591 * t30127;
    let t119468 = t2179 * t27191 * t1053;
    let t119470 = t9276 * t30130;
    let t119473 = t2179 * t5968 * t4805;
    let t119475 = -t104627 - 4.0 * t119443 - 2.0 * t119445 + 4.0 * t119447 - 12.0 * t119450 - 12.0 * t119452 - t104632 - 2.0 / 3.0 * t1349 * t28 * t26791 * t26533 - 2.0 / 3.0 * t1349 * t28 * t26791 * t26538 + t78726 * t2 * t4 * t26 * t1362 / 6.0 + 8.0 * t119468 + 8.0 * t119470 + 4.0 * t119473;
    (t119443, t119445, t119447, t119450, t119452, t119468, t119470, t119473, t119475)
}
