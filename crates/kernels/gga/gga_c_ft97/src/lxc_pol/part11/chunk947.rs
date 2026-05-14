//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 947/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk947<F: Float>(t39370: F, t738: F, t2487: F, t41468: F, t1775: F, t9955: F, t675: F, t9567: F, t2: F, t9925: F, t13313: F, t2373: F, t2459: F, t2486: F, t2493: F, t3910: F, t41454: F, t41473: F, t41817: F, t41857: F, t41861: F, t41865: F, t41930: F, t41940: F, t41945: F, t462: F, t737: F, t9707: F, t9896: F, t9916: F) -> (F, F, F, F) {
    let t42145 = t738 * t39370;
    let t42154 = t2487 * t41468;
    let t42158 = t1775 * t9955;
    let t42163 = t9567 * t675;
    let t42164 = t42163 * t2;
    let t42168 = t1775 * t9925;
    let t42191 = -t462 * t737 * t42145 / 3.0 - 36.0 * t462 * t9707 * t2 * t2373 * t2459 - 2.0 / 3.0 * t462 * t2486 * t42154 + 40.0 / 81.0 * t42158 + 4.0 / 3.0 * t462 * t9916 * t41861 + 40.0 / 27.0 * t462 * t42164 * t41817 + 8.0 / 9.0 * t42168 + 4.0 / 3.0 * t462 * t2493 * t41945 - 8.0 / 9.0 * t462 * t3910 * t41473 - 20.0 / 9.0 * t462 * t13313 * t41454 + 4.0 / 3.0 * t462 * t2493 * t41930 + 2.0 * t462 * t2493 * t41857 + 8.0 * t462 * t2493 * t41865 + 8.0 * t462 * t9896 * t41940;
    (t42145, t42154, t42163, t42191)
}
