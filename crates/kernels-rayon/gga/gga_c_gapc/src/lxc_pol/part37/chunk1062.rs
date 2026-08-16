//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1062/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1062(t12580: f64, t12584: f64, t209: f64, t3903: f64, t575: f64, t687: f64, t1611: f64, t3909: f64, t1616: f64, t11304: f64, t11306: f64, t11309: f64, t11330: f64, t11363: f64, t11367: f64, t12068: f64, t12069: f64, t12070: f64, t12071: f64, t12073: f64, t12074: f64, t12075: f64, t12076: f64, t12077: f64, t12078: f64, t12079: f64, t12080: f64, t12083: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12585 = t12580 + t12584;
    let t12586 = t12585 * t209;
    let t12587 = t3903 * t575;
    let t12588 = t12587 * t687;
    let t12589 = t1611 * t3909;
    let t12590 = t3909 * t687;
    let t12591 = t1616 * t12590;
    let t12592 = 2.0_f64 * t12591;
    let t12599 = 0.18115908419564701086e-6_f64 * t11304 - 0.18115908419564701086e-6_f64 * t11306 + 0.6629778687778673199e-7_f64 * t11309 + t12068 + t12069 - t12070 - t12071 + 0.6629778687778673199e-7_f64 * t11330 - t12073 - t12074 - t12075 + t12076 - t12077 - t12078 + t12079 - t12080 + 0.78584976712469872988e-8_f64 * t11363 - 0.52838066223730378166e-7_f64 * t11367 - t12083;
    (t12585, t12586, t12587, t12588, t12589, t12590, t12591, t12592, t12599)
}
