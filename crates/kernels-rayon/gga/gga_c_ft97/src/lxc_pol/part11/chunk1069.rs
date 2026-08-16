//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1069/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1069(t39370: f64, t738: f64, t2487: f64, t41468: f64, t1775: f64, t9955: f64, t675: f64, t9567: f64, t2: f64, t9925: f64, t13313: f64, t2373: f64, t2459: f64, t2486: f64, t2493: f64, t3910: f64, t41454: f64, t41473: f64, t41817: f64, t41857: f64, t41861: f64, t41865: f64, t41930: f64, t41940: f64, t41945: f64, t462: f64, t737: f64, t9707: f64, t9896: f64, t9916: f64) -> (f64, f64, f64, f64) {
    let t42145 = t738 * t39370;
    let t42154 = t2487 * t41468;
    let t42158 = t1775 * t9955;
    let t42163 = t9567 * t675;
    let t42164 = t42163 * t2;
    let t42168 = t1775 * t9925;
    let t42191 = -t462 * t737 * t42145 / 3.0_f64 - 36.0_f64 * t462 * t9707 * t2 * t2373 * t2459 - 2.0_f64 / 3.0_f64 * t462 * t2486 * t42154 + 40.0_f64 / 81.0_f64 * t42158 + 4.0_f64 / 3.0_f64 * t462 * t9916 * t41861 + 40.0_f64 / 27.0_f64 * t462 * t42164 * t41817 + 8.0_f64 / 9.0_f64 * t42168 + 4.0_f64 / 3.0_f64 * t462 * t2493 * t41945 - 8.0_f64 / 9.0_f64 * t462 * t3910 * t41473 - 20.0_f64 / 9.0_f64 * t462 * t13313 * t41454 + 4.0_f64 / 3.0_f64 * t462 * t2493 * t41930 + 2.0_f64 * t462 * t2493 * t41857 + 8.0_f64 * t462 * t2493 * t41865 + 8.0_f64 * t462 * t9896 * t41940;
    (t42145, t42154, t42163, t42191)
}
