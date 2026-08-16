//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 765/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk765(t185: f64, t8998: f64, t1723: f64, t563: f64, t595: f64, t1941: f64, t8972: f64, t8974: f64, t8976: f64, t8978: f64, t8980: f64, t8982: f64, t8984: f64, t8988: f64, t8990: f64, t8994: f64, t8996: f64) -> (f64, f64) {
    let t8999 = t185 * t8998;
    let t9000 = t8999 * t1723;
    let t9002 = t563 * t595;
    let t9003 = t9002 * t1941;
    let t9005 = 0.15176747947735985782e-6_f64 * t8972 - 0.26984257851074582721e-6_f64 * t8974 + 0.21642471925239962898e-3_f64 * t8976 - 0.21642471925239962898e-3_f64 * t8978 - 0.20611878024038059902e-5_f64 * t8980 + 0.36647919126739670507e-5_f64 * t8982 + 0.12380568050579229813e-5_f64 * t8984 + 0.80045999977926802213e-7_f64 * t8988 + 0.27801896084645508334e-2_f64 * t8990 + 0.9275345110817126956e-4_f64 * t8994 + 0.77294542590142724635e-6_f64 * t8996 - 0.1374296967252737644e-5_f64 * t9000 - 0.12357942809624928455e-3_f64 * t9003;
    (t8999, t9005)
}
