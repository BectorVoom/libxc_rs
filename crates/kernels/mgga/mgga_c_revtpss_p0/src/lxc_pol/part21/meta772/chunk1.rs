//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2743/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2743<F: Float>(t125: F, t14662: F, t10777: F, t14671: F, t14917: F, t40725: F, t10811: F, t14678: F, t10871: F, t1558: F, t10627: F, t10639: F, t10666: F, t10786: F, t14767: F, t14785: F, t14791: F, t14894: F, t1544: F, t221: F, t2722: F, t2745: F, t2747: F, t40438: F, t40440: F, t40462: F, t4362: F, t4364: F, t4365: F, t4366: F, t4450: F, t50409: F, t50415: F, t50418: F, t50423: F, t50436: F, t50443: F, t50446: F, t50454: F, t50457: F, t775: F, t828: F, t837: F, t851: F) -> (F, F, F) {
    let t50459 = t125 * t14662;
    let t50466 = t10777 * t40725 * t14671 * t14917;
    let t50472 = t10811 * t14678;
    let t50474 = t1558 * t10871;
    let t50480 = -F::cast_from(0.60023625365297631762e-2_f64) * t50409 - F::cast_from(0.38115002106963996168e-4_f64) * t40438 + F::cast_from(0.15246000842785598468e-3_f64) * t50415 - F::cast_from(0.12004725073059526352e-1_f64) * t40440 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t14791 * t50418 * t837 + F::cast_from(0.25724410870841842184e-1_f64) * t4362 * t14785 * t50423 * t10786 - F::cast_from(0.21437009059034868486e-3_f64) * t2745 * t4364 * t4365 * t10639 - F::cast_from(0.20082057720118594944e-6_f64) * t50436 + F::cast_from(0.18007087609589289528e0_f64) * t851 * t40462 * t828 * t1544 * t10627 + F::cast_from(0.45732285992607719437e-3_f64) * t50443 - F::new(3.0) / F::new(4.0) * t50446 * t221 * t14767 * t775 - t50454 + F::cast_from(0.27107389498472794076e-3_f64) * t50457 + F::cast_from(0.12862205435420921092e-2_f64) * t4362 * t4364 * t50459 * t4366 - F::cast_from(0.76230004213927992338e-3_f64) * t50466 + F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t2747 * t4450 * t10666 + F::cast_from(0.60023625365297631762e-2_f64) * t50472 + F::cast_from(0.1543464652250510531e-1_f64) * t14894 * t14791 * t50474 * t2722 * t775;
    (t50459, t50474, t50480)
}
