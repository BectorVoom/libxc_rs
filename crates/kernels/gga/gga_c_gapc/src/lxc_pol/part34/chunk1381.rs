//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1381/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1381<F: Float>(t33784: F, t33787: F, t33789: F, t33791: F, t33793: F, t33796: F, t33779: F, t36698: F, t36699: F, t36700: F, t36701: F, t33801: F, t33803: F, t33808: F, t33810: F, t33812: F, t33815: F, t33818: F, t33820: F, t33823: F, t33825: F, t33828: F) -> (F, F) {
    let t36703 = F::new(0.19336232562226912508e-8) * t33784;
    let t36704 = F::new(0.2845640240200497334e-7) * t33787;
    let t36705 = F::new(0.34782544165564226085e-4) * t33789;
    let t36706 = F::new(0.42205124476153752644e-7) * t33791;
    let t36707 = F::new(0.33764099580923002116e-6) * t33793;
    let t36708 = F::new(0.21102562238076876322e-7) * t33796;
    let t36709 = -t36698 - t36699 - t36700 + t36701 - F::new(0.57970906942607043474e-5) * t33779 - t36703 + t36704 + t36705 - t36706 + t36707 + t36708;
    let t36722 = F::new(0.40094868252346065012e-6) * t33801 - F::new(0.21102562238076876322e-7) * t33803 - F::new(0.22098551499687900008e-7) * t33808 - F::new(0.55015711310542948459e-6) * t33810 + F::new(0.40481770833333333336e-4) * t33812 + F::new(0.57920616843011475696e-5) * t33815 - F::new(0.50680539737635041234e-3) * t33818 - F::new(0.34752370105806885418e-3) * t33820 + F::new(0.57920616843011475696e-5) * t33823 - F::new(0.50680539737635041234e-3) * t33825 - F::new(0.34752370105806885418e-3) * t33828;
    (t36709, t36722)
}
