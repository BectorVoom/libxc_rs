//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2743/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2743(t125: f64, t14662: f64, t10777: f64, t14671: f64, t14917: f64, t40725: f64, t10811: f64, t14678: f64, t10871: f64, t1558: f64, t10627: f64, t10639: f64, t10666: f64, t10786: f64, t14767: f64, t14785: f64, t14791: f64, t14894: f64, t1544: f64, t221: f64, t2722: f64, t2745: f64, t2747: f64, t40438: f64, t40440: f64, t40462: f64, t4362: f64, t4364: f64, t4365: f64, t4366: f64, t4450: f64, t50409: f64, t50415: f64, t50418: f64, t50423: f64, t50436: f64, t50443: f64, t50446: f64, t50454: f64, t50457: f64, t775: f64, t828: f64, t837: f64, t851: f64) -> (f64, f64, f64) {
    let t50459 = t125 * t14662;
    let t50466 = t10777 * t40725 * t14671 * t14917;
    let t50472 = t10811 * t14678;
    let t50474 = t1558 * t10871;
    let t50480 = -0.60023625365297631762e-2_f64 * t50409 - 0.38115002106963996168e-4_f64 * t40438 + 0.15246000842785598468e-3_f64 * t50415 - 0.12004725073059526352e-1_f64 * t40440 + 0.25724410870841842183e-2_f64 * t2745 * t14791 * t50418 * t837 + 0.25724410870841842184e-1_f64 * t4362 * t14785 * t50423 * t10786 - 0.21437009059034868486e-3_f64 * t2745 * t4364 * t4365 * t10639 - 0.20082057720118594944e-6_f64 * t50436 + 0.18007087609589289528e0_f64 * t851 * t40462 * t828 * t1544 * t10627 + 0.45732285992607719437e-3_f64 * t50443 - 3.0_f64 / 4.0_f64 * t50446 * t221 * t14767 * t775 - t50454 + 0.27107389498472794076e-3_f64 * t50457 + 0.12862205435420921092e-2_f64 * t4362 * t4364 * t50459 * t4366 - 0.76230004213927992338e-3_f64 * t50466 + 0.85748036236139473944e-3_f64 * t2745 * t2747 * t4450 * t10666 + 0.60023625365297631762e-2_f64 * t50472 + 0.1543464652250510531e-1_f64 * t14894 * t14791 * t50474 * t2722 * t775;
    (t50459, t50474, t50480)
}
