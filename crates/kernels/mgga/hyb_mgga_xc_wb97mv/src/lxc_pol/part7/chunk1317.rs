//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1317/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1317<F: Float>(t11421: F, t967: F, t2555: F, t4327: F, t11395: F, t11400: F, t11432: F, t11532: F, t1416: F, t1428: F, t23379: F, t23447: F, t23456: F, t2529: F, t2551: F, t2559: F, t2568: F, t2598: F, t27538: F, t27647: F, t32087: F, t32104: F, t32116: F, t32130: F, t32143: F, t3544: F, t3564: F, t3577: F, t3597: F, t4333: F, t4346: F, t4349: F, t4360: F, t4373: F, t7249: F, t7254: F, t7322: F, t9452: F, t9465: F, t9493: F, t9496: F, t968: F, t976: F, t977: F) -> (F,) {
    let t32159 = t11421 * t967;
    let t32164 = t4327 * t2555;
    let t32181 = 0.17315859105681463759e2 * t32087 * t2598 + 0.11696447245269292414e1 * t27647 * t1428 + 0.23392894490538584828e1 * t9452 * t3597 + 1.0 * t968 * (t32104 + t32116 + t32130 + t32143) * t976 + 0.11696447245269292414e1 * t3577 * t9465 - 0.11696447245269292414e1 * t23447 * t4360 + 0.5848223622634646207e0 * t7249 * t4373 + 0.11696447245269292414e1 * t2568 * t11395 + 0.32163958997385070134e2 * t23379 * t4349 + 2.0 * t32159 * t977 + 1.0 * t11400 * t2551 + 0.32163958997385070134e2 * t32164 * t2559 + 2.0 * t27538 * t1416 + 4.0 * t9496 * t3564 + 2.0 * t3544 * t9493 - 2.0 * t23456 * t4333 + 1.0 * t7322 * t4346 + 2.0 * t2529 * t11532 + 0.70178683471615754484e1 * t7254 * t11432;
    (t32181,)
}
