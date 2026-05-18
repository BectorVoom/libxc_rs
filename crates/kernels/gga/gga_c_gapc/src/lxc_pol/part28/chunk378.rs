//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 378/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk378<F: Float>(t1703: F, t185: F, t1622: F, t1626: F, t1633: F, t1638: F, t1643: F, t1650: F, t1653: F, t1659: F, t1662: F, t1668: F, t1674: F, t1677: F, t1679: F, t1682: F, t1687: F, t1691: F, t1696: F, t1700: F, t1704: F, t1708: F, t1711: F, t1714: F, t1717: F, t1723: F, t179: F, t459: F, t569: F, t590: F, t596: F, t600: F, t669: F) -> (F, F) {
    let t1726 = t185 * t1703;
    let t1729 = F::new(0.687148483626368822e-7) * t1622 * t1626 - F::new(0.91631250291576282414e-7) * t1633 * t1626 - F::new(0.37073828428874785365e-3) * t596 * t1638 + F::new(0.59127296360574214771e-4) * t1643 * t1650 + F::new(0.69504740211613770836e-4) * t1653 * t459 + F::new(0.69504740211613770836e-4) * t590 * t569 + F::new(0.12357942809624928455e-3) * t596 * t1659 + F::new(0.12357942809624928455e-3) * t1662 * t600 + F::new(0.67632724766374884053e-5) * t669 * t1668 - F::new(0.67632724766374884053e-5) * t669 * t1674 + F::new(0.33816362383187442026e-5) * t1677 * t1679 + F::new(0.6487109086417285278e-2) * t179 * t1682 - F::new(0.2318836277704281739e-4) * t1687 * t1691 - F::new(0.19323635647535681158e-6) * t1696 * t1700 + F::new(0.343574241813184411e-6) * t1704 * t1700 + F::new(0.27801896084645508334e-2) * t1708 * t1711 - F::new(0.38647271295071362317e-6) * t1714 * t1717 + F::new(0.38647271295071362317e-6) * t1714 * t1723 - F::new(0.687148483626368822e-6) * t1726 * t1723;
    (t1726, t1729)
}
