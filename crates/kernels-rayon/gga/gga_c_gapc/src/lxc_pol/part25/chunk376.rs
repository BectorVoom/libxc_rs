//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 376/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk376(t1703: f64, t185: f64, t1622: f64, t1626: f64, t1633: f64, t1638: f64, t1643: f64, t1650: f64, t1653: f64, t1659: f64, t1662: f64, t1668: f64, t1674: f64, t1677: f64, t1679: f64, t1682: f64, t1687: f64, t1691: f64, t1696: f64, t1700: f64, t1704: f64, t1708: f64, t1711: f64, t1714: f64, t1717: f64, t1723: f64, t179: f64, t459: f64, t569: f64, t590: f64, t596: f64, t600: f64, t669: f64) -> (f64, f64) {
    let t1726 = t185 * t1703;
    let t1729 = 0.687148483626368822e-7_f64 * t1622 * t1626 - 0.91631250291576282414e-7_f64 * t1633 * t1626 - 0.37073828428874785365e-3_f64 * t596 * t1638 + 0.59127296360574214771e-4_f64 * t1643 * t1650 + 0.69504740211613770836e-4_f64 * t1653 * t459 + 0.69504740211613770836e-4_f64 * t590 * t569 + 0.12357942809624928455e-3_f64 * t596 * t1659 + 0.12357942809624928455e-3_f64 * t1662 * t600 + 0.67632724766374884053e-5_f64 * t669 * t1668 - 0.67632724766374884053e-5_f64 * t669 * t1674 + 0.33816362383187442026e-5_f64 * t1677 * t1679 + 0.6487109086417285278e-2_f64 * t179 * t1682 - 0.2318836277704281739e-4_f64 * t1687 * t1691 - 0.19323635647535681158e-6_f64 * t1696 * t1700 + 0.343574241813184411e-6_f64 * t1704 * t1700 + 0.27801896084645508334e-2_f64 * t1708 * t1711 - 0.38647271295071362317e-6_f64 * t1714 * t1717 + 0.38647271295071362317e-6_f64 * t1714 * t1723 - 0.687148483626368822e-6_f64 * t1726 * t1723;
    (t1726, t1729)
}
