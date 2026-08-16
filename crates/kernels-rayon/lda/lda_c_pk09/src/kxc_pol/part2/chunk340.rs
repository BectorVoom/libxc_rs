//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 340/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk340(t1504: f64, t297: f64, t305: f64, t317: f64, t1336: f64, t327: f64, t1625: f64, t304: f64, t1520: f64, t1526: f64, t1268: f64, t1275: f64, t1451: f64, t1514: f64, t1517: f64, t1522: f64, t1621: f64, t1627: f64, t1629: f64, t307: f64, t319: f64, t328: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1632 = t1504 * t297;
    let t1633 = t317 * t305;
    let t1634 = t1632 * t1633;
    let t1637 = t327 * t1336;
    let t1639 = t1637 * t1625 / 6.0_f64;
    let t1642 = t304 * t1336;
    let t1644 = t1642 * t1625 / 6.0_f64;
    let t1649 = 0.14975624337724558_f64 * t1520;
    let t1651 = 0.037002892246025966_f64 * t1526;
    let t1652 = -0.10237773105191754_f64 * t1275 - t1621 + 0.10237773105191754_f64 * t1268 + t1627 + t319 * t1629 / 6.0_f64 - t1634 * t1451 / 6.0_f64 - t1639 - t328 * t1629 / 6.0_f64 + t1644 + t307 * t1629 / 6.0_f64 + 0.037002892246025966_f64 * t1514 - 0.037002892246025966_f64 * t1517 - t1649 - 0.14975624337724558_f64 * t1522 - t1651;
    (t1632, t1633, t1634, t1637, t1639, t1642, t1644, t1649, t1651, t1652)
}
