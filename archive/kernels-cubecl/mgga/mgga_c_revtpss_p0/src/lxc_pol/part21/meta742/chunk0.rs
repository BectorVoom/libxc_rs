//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2613/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2613<F: Float>(t48089: F, t221: F, t9817: F, t13792: F, t13845: F, t1882: F, t9994: F, t13793: F, t13999: F, t1868: F, t3923: F, t1353: F, t13783: F, t13789: F, t13790: F, t13791: F, t13804: F, t1398: F, t21990: F, t3889: F, t46592: F, t46596: F, t46598: F, t46600: F, t46602: F, t46607: F, t46613: F, t46618: F, t46620: F, t46622: F, t46633: F, t46641: F, t5671: F, t9835: F) -> (F, F, F, F, F) {
    let t48090 = F::cast_from(0.34697458558045176417e-2_f64) * t48089;
    let t48100 = t9817 * t221;
    let t48102 = t13845 * t48100 * t13792;
    let t48105 = t1882 * t9994;
    let t48111 = t13999 * t13793;
    let t48113 = t1868 * t3923;
    let t48129 = -F::cast_from(0.10289764348336736873e-1_f64) * t5671 * t13789 * t21990 * t13791 - F::cast_from(0.51448821741683684367e-2_f64) * t5671 * t13789 * t13790 * t3889 * t1398 - F::cast_from(0.6098400337114239387e-3_f64) * t48102 - F::cast_from(0.15246000842785598467e-3_f64) * t46592 + F::cast_from(0.1543464652250510531e-1_f64) * t13804 * t13789 * t48105 * t3923 * t1353 + F::cast_from(0.48018900292238105409e-1_f64) * t48111 + F::cast_from(0.25724410870841842184e-1_f64) * t5671 * t13783 * t48113 * t9835 + F::cast_from(0.45351183609335988443e0_f64) * t46596 - F::cast_from(0.22866142996303859718e-3_f64) * t46598 + F::cast_from(0.40015750243531754508e-2_f64) * t46600 + F::cast_from(0.16262400898971305031e-2_f64) * t46602 - F::cast_from(0.42874018118069736972e-3_f64) * t46607 + F::cast_from(0.42874018118069736972e-4_f64) * t46613 + F::cast_from(0.71456696863449561619e-5_f64) * t46618 + F::cast_from(0.21675198048579700358e-2_f64) * t46620 + F::cast_from(0.12004725073059526352e0_f64) * t46622 + F::cast_from(0.34013387707001991332e0_f64) * t46633 - F::cast_from(0.50820002809285328224e-4_f64) * t46641;
    (t48090, t48100, t48105, t48113, t48129)
}
