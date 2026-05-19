//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1042/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1042<F: Float>(t238: F, t108826: F, t122830: F, t123445: F, t140884: F, t140885: F, t141075: F, t141082: F, t141097: F, t141112: F, t141117: F, t141160: F, t150321: F, t150328: F, t150331: F, t150336: F, t150344: F, t150393: F, t150428: F, t150460: F, t150494: F, t150537: F, t150577: F, t150611: F, t150618: F, t150621: F, t150625: F, t150630: F, t150637: F, t150640: F, t150684: F, t150722: F, t150761: F, t150797: F, t150831: F, t150875: F, t150907: F, t17842: F, t17864: F, t27501: F, t27515: F, t27529: F, t27543: F, t27548: F, t27552: F, t27557: F, t27566: F, t27621: F, t27658: F, t27679: F, t30671: F, t33356: F, t33380: F, t33414: F, t33424: F, t33426: F, t35384: F, t36792: F, t3771: F, t684: F) -> F {
    let t239 = F::new(0.1e-59) < t238;
    let t150912 = piecewise3::<F>(t239, -F::cast_from(0.78259321553885081522e-2_f64) * t150328 * t150331 + F::cast_from(0.65216101294904234602e-2_f64) * t150328 * t150336 - F::cast_from(0.10338048737805743097e-3_f64) * t108826 * t27543 - F::cast_from(0.90822088511484663583e-3_f64) * t27658 * t150321 + F::cast_from(0.21120586720831816187e-4_f64) * t150637 * t27621 + F::cast_from(0.22227677429409423704e-2_f64) * t30671 * t150630 - F::cast_from(0.17816121467177433866e-2_f64) * t141112 * t27679 - F::cast_from(0.11738898233082762229e-1_f64) * t141160 * t33426 * t150344 * t684 - F::cast_from(0.90845139567911167717e-8_f64) * t3771 * t150618 * t35384 * t150621 - F::cast_from(0.55136259934963963187e-3_f64) * t27566 * t33356 * t17842 - F::cast_from(0.79202200203119310706e-5_f64) * t150640 * t36792 * t27529 + F::cast_from(0.79202200203119310706e-5_f64) * t141117 * t36792 * t17864 + F::cast_from(0.20474018672298993868e-3_f64) * t140884 * t140885 * t27552 + F::cast_from(0.13649345781532662579e-4_f64) * t33424 * t140885 * t27557 + F::cast_from(0.51074886703703703704e-1_f64) * t33380 * t141097 * t27501 + F::cast_from(0.20434969915084049306e-2_f64) * t123445 * t33414 * t27515 - F::cast_from(0.13623313276722699538e-2_f64) * t122830 * t33414 * t27548 + F::cast_from(0.51074886703703703704e-1_f64) * t141082 - F::cast_from(0.22705522127871165896e-3_f64) * t141075 + t150460 + t150428 + t150494 + t150684 + t150797 + t150831 + t150537 + t150761 + t150722 + t150577 + t150393 + t150907 + t150875 + t150611 + F::cast_from(0.51074886703703703703e-1_f64) * t150625, F::new(0.0));
    t150912
}
