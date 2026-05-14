//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1284/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1284<F: Float>(t23405: F, t27423: F, t27429: F, t11176: F, t1348: F, t26811: F, t40524: F, t6708: F, t26523: F, t9276: F, t1053: F, t2179: F, t23938: F, t13070: F, t1953: F, t23410: F, t24061: F, t24139: F, t26817: F, t40830: F, t5772: F, t5773: F, t6580: F, t6584: F, t6723: F, t94269: F, t94363: F, t94984: F) -> (F, F, F, F) {
    let t104552 = 2.0 / 27.0 * t23405 * t27423;
    let t104554 = 2.0 / 81.0 * t23405 * t27429;
    let t104562 = t1348 * t11176 * t26811;
    let t104566 = t40524 * t6708;
    let t104575 = t9276 * t26523;
    let t104578 = t2179 * t23938 * t1053;
    let t104580 = -t104552 + t104554 - 2.0 / 3.0 * t6580 * t24061 - 4.0 * t5772 * t40830 * t5773 * t13070 + 11.0 / 27.0 * t104562 - t94363 / 9.0 - t1953 * t6723 + 4.0 * t104566 - t26817 * t23410 / 9.0 - t26817 * t24139 / 27.0 - t94269 * t6584 / 18.0 - 4.0 / 81.0 * t94984 + 8.0 * t104575 + 4.0 * t104578;
    (t104566, t104575, t104578, t104580)
}
