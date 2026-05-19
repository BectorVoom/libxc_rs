//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 340/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk340<F: Float>(t1504: F, t297: F, t305: F, t317: F, t1336: F, t327: F, t1625: F, t304: F, t1520: F, t1526: F, t1268: F, t1275: F, t1451: F, t1514: F, t1517: F, t1522: F, t1621: F, t1627: F, t1629: F, t307: F, t319: F, t328: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1632 = t1504 * t297;
    let t1633 = t317 * t305;
    let t1634 = t1632 * t1633;
    let t1637 = t327 * t1336;
    let t1639 = t1637 * t1625 / F::new(6.0);
    let t1642 = t304 * t1336;
    let t1644 = t1642 * t1625 / F::new(6.0);
    let t1649 = F::cast_from(0.14975624337724558_f64) * t1520;
    let t1651 = F::cast_from(0.037002892246025966_f64) * t1526;
    let t1652 = -F::cast_from(0.10237773105191754_f64) * t1275 - t1621 + F::cast_from(0.10237773105191754_f64) * t1268 + t1627 + t319 * t1629 / F::new(6.0) - t1634 * t1451 / F::new(6.0) - t1639 - t328 * t1629 / F::new(6.0) + t1644 + t307 * t1629 / F::new(6.0) + F::cast_from(0.037002892246025966_f64) * t1514 - F::cast_from(0.037002892246025966_f64) * t1517 - t1649 - F::cast_from(0.14975624337724558_f64) * t1522 - t1651;
    (t1632, t1633, t1634, t1637, t1639, t1642, t1644, t1649, t1651, t1652)
}
